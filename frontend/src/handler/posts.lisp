(uiop:define-package #:frontend.handler.posts
  (:use #:cl)
  (:import-from #:frontend.handler
                #:set-status #:set-header #:set-body
                #:get-cookie)
  (:import-from #:frontend.service.posts
                #:retrieve-posts)
  (:import-from #:frontend.template.posts
                #:render-posts)
  (:export #:*get-posts*))

(in-package #:frontend.handler.posts)

(defparameter *get-posts*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((pagination (dto:params->pagination-params params))
                 (sorting (dto:params->sort-posts-params params))
                 (posts (retrieve-posts pagination sorting)))
            (set-status 200)
            (set-header :content-type "text/html; charset=utf-8")
            (set-body (template:with-page (:title "Posts" :userp (not (null (get-cookie "token"))))
                        (let ((sort-by (dto:sort-posts-params-sort-by sorting))
                              (sort-order (dto:sort-posts-params-sort-order sorting))
                              (offset (dto:pagination-params-offset pagination))
                              (limit (dto:pagination-params-limit pagination)))
                          (render-posts posts sort-by sort-order offset limit)))))
        (dto:bad-input (c)
          (handler:report-error (:title "Error" :message (dto:bad-input-reason c))))
        (service:backend-error (c)
          (handler:report-error (:title "Error" :message (service:backend-error-message c))))))))
