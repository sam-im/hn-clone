(uiop:define-package #:frontend.service.user
  (:use #:cl)
  (:import-from #:frontend.service
                #:backend-error
                #:backend-error-message)
  (:import-from #:com.inuoe.jzon
                #:stringify
                #:parse)
  (:export #:login
           #:logout
           #:register))

(in-package #:frontend.service.user)

(defun login (form)
  (let ((request-body (stringify form)))
    (handler-case
        (parse (dex:post (format nil "~a/session" service:*backend-url*)
                         :headers '(("Content-Type" . "application/json"))
                         :content request-body))
      (dex:http-request-failed (c)
        (error 'backend-error :status (dex:response-status c)
                              :message (gethash "error_message" (parse (dex:response-body c))))))))

(defun logout (token)
  (handler-case
      (dex:delete (format nil "~a/session/~a" service:*backend-url* token))
    (dex:http-request-failed (c)
      (error 'backend-error :status (dex:response-status c)
                            :message (gethash "error_message" (parse (dex:response-body c)))))))

(defun register (form)
  (let ((request-body (stringify form)))
    (handler-case
        (dex:post (format nil "~a/user" service:*backend-url*)
                  :headers '(("Content-Type" . "application/json"))
                  :content request-body)
      (dex:http-request-failed (c)
        (error 'backend-error :status (dex:response-status c)
                              :message (gethash "error_message" (parse (dex:response-body c))))))))
