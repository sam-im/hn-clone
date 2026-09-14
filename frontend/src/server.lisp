(uiop:define-package frontend.server
  (:use #:cl)
  (:import-from :frontend.handler.login
                #:*get-login*
                #:*post-login*
                #:*get-logout*)
  (:import-from :frontend.handler.register
                #:*get-register*
                #:*post-register*)
  (:export #:make-app))
(in-package #:frontend.server)

(defun make-app ()
  (let ((app (make-instance 'ningle:app)))
    ;; (setf (ningle:route app "/") *get-root*)
    (setf (ningle:route app "/login") *get-login*)
    (setf (ningle:route app "/login" :method :post) *post-login*)
    (setf (ningle:route app "/logout") *get-logout*)
    (setf (ningle:route app "/register") *get-register*)
    (setf (ningle:route app "/register" :method :post) *post-register*)
    (lack:builder
     (:static :path "/static/" :root (asdf:system-relative-pathname :frontend "static/"))
     app)))
