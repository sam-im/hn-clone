(uiop:define-package frontend.service
  (:nicknames service)
  (:use #:cl)
  (:export #:*backend-url*)
  (:export #:backend-error
           #:backend-error-status
           #:backend-error-message))

(in-package #:frontend.service)

(defparameter *backend-url* "http://127.0.0.1:3000")

(define-condition backend-error (error)
  ((status :initarg :status
           :initform nil
           :reader backend-error-status)
   (message :initarg :message
            :initform nil
            :reader backend-error-message)))
