(uiop:define-package frontend.handler.post
  (:use #:cl)
  (:import-from :frontend.handler
                #:set-status #:set-header #:set-cookie #:set-body
                #:get-cookie)
  (:import-from :frontend.template.post
                #:render-post
                #:render-new-post)
  (:import-from :frontend.service.post
                #:retrieve-post
                #:retrieve-comments
                #:create-post)
  (:export #:*get-post*
           #:*post-post*))

(in-package #:frontend.handler.post)

(defparameter *get-post*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((post-id (dto:params->post-id params))
                 (pagination (dto:params->pagination-params params))
                 (post (retrieve-post post-id))
                 (sorting (dto:params->sort-params params))
                 (comments (retrieve-comments post-id pagination sorting)))
            (set-status 200)
            (set-header :content-type "text/html; charset=utf-8")
            (set-body (template:with-page (:title (gethash "title" post)
                                           :userp (not (null (get-cookie "token"))))
                        (render-post post comments))))
        (dto:bad-input (c)
          (handler:report-error (:title "Post Not Found" :message (dto:bad-input-reason c))))
        (service:backend-error (c)
          (let* ((status (service:backend-error-status c))
                 (title (if (or (= status 400) (= status 404))
                            "Post Not Found"
                            "Error")))
            (handler:report-error (:title title :message (service:backend-error-message c)))))))))

(defparameter *get-new-post*
  (lambda (params)
    (declare (ignore params))
    (set-status 200)
    (set-header :content-type "text/html; charset=utf-8")
    (set-body (template:with-page (:title "New Post")
                (render-new-post)))))

(defparameter *post-post*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((token (get-cookie "token"))
                 (post-params (dto:params->post-params params))
                 (post (create-post token post-params)))
            (handler:redirect (format nil "/post/~a" (gethash "id" post))))
        ;; TODO: save datum in bad-input instead of re-extracting it
        (dto:bad-input (c)
          (handler:report-error (:title "New Post" :message (dto:bad-input-reason c))
            (render-new-post (:title (cdr (assoc "title" params :test #'string=))
                              :content (cdr (assoc "content" params :test #'string=))))))
        (service:backend-error (c)
          (if (= 401 (service:backend-error-status c))
              (handler:report-auth-error "Please login to create new posts.")
              (handler:report-error (:title "New Post"
                                     :message (service:backend-error-message c)))))))))
