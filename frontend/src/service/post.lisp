(uiop:define-package #:frontend.service.post
  (:use #:cl)
  (:import-from #:com.inuoe.jzon
                #:parse
                #:stringify)
  (:export #:retrieve-post
           #:retrieve-comments
           #:create-post))

(in-package #:frontend.service.post)

(defun retrieve-post (post-id)
  (handler-case
      (parse (dex:get (format nil "~a/post/~a" service:*backend-url* post-id)))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))

(defun retrieve-comments (post-id pagination sorting)
  (handler-case
      (parse (dex:get (format nil "~a/post/~a/comments?offset=~a&limit=~a&sort_by=~a"
                              service:*backend-url*
                              post-id
                              (dto:pagination-params-offset pagination)
                              (dto:pagination-params-limit pagination)
                              (dto:sort-comments-params-sort-by sorting))))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))

(defun create-post (token post-params)
  ;; HACK: backend returns non-json body if token is missing
  (unless token
    (error 'service:backend-error :status 401 :message "Missing session token."))
  (handler-case
      (parse (dex:post (format nil "~a/post" service:*backend-url*)
                       :headers '(("Content-Type" . "application/json"))
                       :bearer-auth token
                       :content (stringify post-params)))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))
