(uiop:define-package frontend.handler.register
  (:use #:cl)
  (:import-from :frontend.handler
                #:set-status
                #:set-header
                #:set-body)
  (:import-from :frontend.dto
                #:params->register-params)
  (:import-from :frontend.template
                #:with-page
                #:render-error-message)
  (:import-from :frontend.template.register
                #:render-register-form)
  (:import-from :frontend.service.user
                #:register)
  (:export #:*get-register*
           #:*post-register*))

(in-package :frontend.handler.register)

(defparameter *get-register*
  (lambda (params)
    (declare (ignore params))
    (set-status 200)
    (set-header :content-type "text/html; charset=utf-8")
    (set-body (with-page (:title "Register") (render-register-form)))))

(defparameter *post-register*
  (lambda (params)
    (flet ((report-error (msg)
             (set-status 200)
             (set-header :content-type "text/html; charset=utf-8")
             (set-body (with-page (:title "Register")
                         (render-error-message msg)
                         (render-register-form)))))
      (handler-case
          (let* ((form (params->register-params params))
                 (response (register form)))
            (declare (ignore response))
            (set-status 303)
            (set-header :location "/login"))

        (dto:bad-input (c)
          (report-error (dto:bad-input-reason c)))
        (service:backend-error (c)
          (report-error (service:backend-error-message c)))))))
