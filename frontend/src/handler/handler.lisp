(uiop:define-package frontend.handler
  (:nicknames handler)
  (:use #:cl)
  (:export #:set-status
           #:set-header
           #:set-cookie
           #:set-body)
  (:export #:get-cookie)
  (:export #:report-error
           #:report-auth-error)
  (:export #:redirect)
  (:export #:with-error-handler))

(in-package #:frontend.handler)

(defmacro set-status (status)
  "Sets the status code of a ningle:*response*.
STATUS must be an integer representing a valid HTTP status code.
Returns NIL."
  `(progn
     (setf (lack.response:response-status ningle:*response*) ,status)
     nil))

(defmacro set-header (key value)
  "Sets an header on a response to KEY and VALUE.
KEY must be a keyword accepted by lack.response.
Returns NIL."
  `(progn
     (setf (getf (lack.response:response-headers ningle:*response*) ,key)
           ,value)
     nil))

(defmacro set-cookie (name &rest plist)
  "Sets a cookie on a response.
PLIST must be a property list of keywords and values accepted by lack.response.
Returns NIL."
  `(progn
     (setf (getf (lack.response:response-set-cookies ningle:*response*) ,name)
           (list ,@plist))
     nil))

(defmacro set-body (body)
  "Sets the body of a response to BODY.
Returns NIL."
  `(progn
     (setf (lack.response:response-body ningle:*response*) ,body)
     nil))

(defmacro get-cookie (name)
  "Retrieve the cookie specified by NAME."
  `(cdr (assoc ,name (lack.request:request-cookies ningle:*request*) :test #'string=)))

(defmacro report-error ((&key title message) &body body)
  `(progn (set-status 200)
          (set-header :content "text/html; charset=utf-8")
          (set-body (template:with-page (:title ,title)
                      (template:render-error-message ,message)
                      ,@body))))

(defmacro report-auth-error (message)
  `(report-error (:title "Login" :message ,message)
     (frontend.template.login:render-login-form)))

(defmacro redirect (path)
  `(progn (set-status 303)
          (set-header :location ,path)))

(defmacro with-error-handler (&body body)
  `(handler-case
       (progn ,@body)
     (error (c)
       (let* ((error-message (handler-case (princ-to-string c)
                               (error () "<Unable to print condition>")))
              (message (format nil "An unhandled error has occured: ~a" error-message)))
         (print message)
         (report-error (:title "Internal Server Error"
                        :message message))))))
       
