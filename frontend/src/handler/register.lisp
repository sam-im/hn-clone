(uiop:define-package #:frontend.handler.register
  (:use #:cl)
  (:import-from #:frontend.handler
                #:set-status
                #:set-header
                #:set-body)
  (:import-from #:frontend.template.register
                #:render-register-form)
  (:import-from #:frontend.service.user
                #:register)
  (:export #:*get-register*
           #:*post-register*))

(in-package #:frontend.handler.register)

(defparameter *get-register*
  (lambda (params)
    (declare (ignore params))
    (set-status 200)
    (set-header :content-type "text/html; charset=utf-8")
    (set-body (template:with-page (:title "Register") (render-register-form)))))

(defparameter *post-register*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((register-params (dto:params->register-params params))
                 (response (register register-params)))
            (declare (ignore response))
            (set-status 303)
            (set-header :location "/login"))
        (dto:bad-input (c)
          (handler:report-error (:title "Register" :message (dto:bad-input-reason c))
            (render-register-form)))
        (service:backend-error (c)
          (handler:report-error (:title "Register" :message (service:backend-error-message c))
            (render-register-form)))))))
