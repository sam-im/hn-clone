(uiop:define-package #:frontend.service.posts
  (:use #:cl)
  (:import-from #:com.inuoe.jzon
                #:parse)
  (:export #:retrieve-posts))

(in-package #:frontend.service.posts)

(defun retrieve-posts (pagination sorting)
  (handler-case
      (parse (dex:get (format nil "~a/posts?offset=~a&limit=~a&sort_by=~a&sort_order=~a"
                              service:*backend-url*
                              (dto:pagination-params-offset pagination)
                              (dto:pagination-params-limit pagination)
                              (dto:sort-posts-params-sort-by sorting)
                              (dto:sort-posts-params-sort-order sorting))))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))
