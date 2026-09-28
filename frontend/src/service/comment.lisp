(uiop:define-package frontend.service.comment
  (:use #:cl)
  (:import-from :com.inuoe.jzon
                #:stringify
                #:parse)
  (:export #:create-comment
           #:retrieve-comment
           #:retrieve-replies))

(in-package :frontend.service.comment)

(defun create-comment (token comment-params)
  ;; HACK: backend returns non-json body if token is missing
  (unless token
    (error 'service:backend-error :status 401 :message "Missing session token."))
  (handler-case
      (parse (dex:post (format nil "~a/comment" service:*backend-url*)
                       :headers '(("Content-Type" . "application/json"))
                       :bearer-auth token
                       :content (stringify comment-params)))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))

(defun retrieve-comment (comment-id)
  (handler-case
      (parse (dex:get (format nil "~a/comment/~a" service:*backend-url* comment-id)))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))

(defun retrieve-replies (comment-id pagination sorting)
  (handler-case
      (parse (dex:get (format nil "~a/comment/~a/replies?offset=~a&limit=~a&sort_by=~a"
                              service:*backend-url*
                              comment-id
                              (dto:pagination-params-offset pagination)
                              (dto:pagination-params-limit pagination)
                              (dto:sort-comments-params-sort-by sorting))))
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))
