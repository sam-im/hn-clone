(uiop:define-package frontend.service.vote
  (:use #:cl)
  (:import-from :com.inuoe.jzon
                #:parse)
  (:export #:create-vote
           #:delete-vote
           #:toggle-vote))

(in-package :frontend.service.vote)

(defun create-vote (token item-id)
  (handler-case
      (= 201 (nth-value 1 (dex:post (format nil "~a/vote/~a" service:*backend-url* item-id)
                                    :bearer-auth token)))
    (dex:http-request-conflict ()
      nil)
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))

(defun delete-vote (token item-id)
  (handler-case
      (= 204 (nth-value 1 (dex:delete (format nil "~a/vote/~a" service:*backend-url* item-id)
                                      :bearer-auth token)))
    (dex:http-request-not-found ()
      nil)
    (dex:http-request-failed (c)
      (error 'service:backend-error :status (dex:response-status c)
                                    :message (gethash "error_message"
                                                      (parse (dex:response-body c)))))))

(defmacro toggle-vote (token item-id)
  `(progn
     (unless ,token
       (error 'service:backend-error :status 401 :message "Missing session token."))
     (unless (create-vote ,token ,item-id)
       (delete-vote ,token ,item-id))))
