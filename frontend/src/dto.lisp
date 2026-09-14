(uiop:define-package frontend.dto
  (:nicknames dto)
  (:use #:cl)
  (:export #:bad-input
           #:bad-input-reason)
  (:export #:params->login-params
           #:params->register-params))
(in-package :frontend.dto)

(define-condition bad-input (error)
  ((reason :initarg :reason
           :initform nil
           :reader bad-input-reason)))

(defstruct login-params
  (username nil :type string)
  (password nil :type string)
  (duration nil :type integer))

(defun params->login-params (params)
  (let* ((username (cdr (assoc "username" params :test #'string=)))
         (password (cdr (assoc "password" params :test #'string=)))
         (duration-str (cdr (assoc "duration" params :test #'string=)))
         (duration (or (when duration-str
                         (parse-integer duration-str :junk-allowed t))
                       60)))
    (unless (and (stringp username) (stringp password))
      (error 'bad-input :reason "Missing username and/or password."))
    (make-login-params :username username
                       :password password
                       :duration duration)))

(defstruct register-params
  (username nil :type string)
  (password nil :type string))

(defun params->register-params (params)
  (let ((username (cdr (assoc "username" params :test #'string=)))
        (password (cdr (assoc "password" params :test #'string=)))
        (re-password (cdr (assoc "re-password" params :test #'string=))))
    (unless (and (stringp username) (stringp password))
      (error 'bad-input :reason "Missing username and/or password."))
    (unless (string= password re-password)
      (error 'bad-input :reason "Passwords do not match."))
    (make-register-params :username username :password password)))
