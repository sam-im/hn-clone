(uiop:define-package #:frontend.handler.comment
  (:use #:cl)
  (:import-from #:frontend.handler
                #:set-status #:set-header #:set-body
                #:get-cookie)
  (:import-from #:frontend.service.comment
                #:create-comment
                #:retrieve-comment
                #:retrieve-replies)
  (:import-from #:frontend.template.comment
                #:render-comment)
  (:export #:*get-comment*
           #:*post-comment*))

(in-package #:frontend.handler.comment)

(defparameter *get-comment*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((id (dto:params->id params))
                 (comment (retrieve-comment id))
                 (pagination (dto:params->pagination-params params))
                 (sorting (dto:params->sort-comments-params params))
                 (replies (retrieve-replies id pagination sorting)))
            (set-status 200)
            (set-header :content-type "text/html; charset=utf-8")
            (set-body (template:with-page
                          (:title (format nil "Comment: ~a..."
                                          (let ((content (gethash "content" comment)))
                                            (if (< (length content) 20)
                                                content
                                                (subseq content 0 20))))
                           :userp (not (null (get-cookie "token"))))
                        (render-comment comment
                                        replies
                                        (format nil "/comment/~a?offset=~a&limit=~a&sort-by=~a"
                                                id
                                                (dto:pagination-params-offset pagination)
                                                (dto:pagination-params-limit pagination)
                                                (dto:sort-comments-params-sort-by sorting))))))
        (dto:bad-input (c)
          (handler:report-error (:title "Comment Not Found" :message (dto:bad-input-reason c))))
        (service:backend-error (c)
          (let* ((status (service:backend-error-status c))
                 (title (if (or (= status 400) (= status 404))
                            "Comment Not Found"
                            "Error")))
            (handler:report-error (:title title :message (service:backend-error-message c)))))))))

(defparameter *post-comment*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((token (get-cookie "token"))
                 (comment-params (dto:params->comment-params params))
                 (parent-type (dto:params->comment-parent-type params))
                 (comment (create-comment token comment-params)))
            (handler:redirect (format nil "/~a/~a?sort-by=newest#item-~a"
                                      parent-type
                                      (gethash "parent" comment)
                                      (gethash "id" comment))))
        (dto:bad-input (c)
          (handler:report-error (:title "Error" :message (dto:bad-input-reason c))))
        (service:backend-error (c)
          (if (= 401 (service:backend-error-status c))
              (handler:report-auth-error "Please login to comment or reply.")
              (handler:report-error (:title "Error"
                                     :message (service:backend-error-message c)))))))))
