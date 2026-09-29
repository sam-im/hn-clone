(uiop:define-package #:frontend.handler.login
  (:use #:cl)
  (:import-from #:frontend.handler
                #:set-status #:set-header #:set-cookie #:set-body
                #:get-cookie)
  (:import-from #:frontend.template.login
                #:render-login-form)
  (:import-from #:frontend.service.user
                #:login
                #:logout)
  (:export #:*get-login*
           #:*post-login*
           #:*get-logout*))

(in-package #:frontend.handler.login)

(defparameter *get-login*
  (lambda (params)
    (declare (ignore params))
    (set-status 200)
    (set-header :content-type "text/html; charset=utf-8")
    (set-body (template:with-page (:title "Login")
                (render-login-form)))))

;; TODO: accept an optional parameter for a return page/location
(defparameter *post-login*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((login-params (dto:params->login-params params))
                 (response (login login-params))
                 (token (gethash "token" response))
                 (expires (+ (get-universal-time) (- (gethash "expires_at" response)
                                                     (gethash "issued_at" response)))))
            (set-status 303)
            (set-header :location "/")
            (set-cookie "token"
                        :value token
                        :path "/"
                        :secure nil
                        :httponly t
                        :samesite :strict
                        :expires expires))
        (dto:bad-input (c)
          (handler:report-auth-error (dto:bad-input-reason c)))
        (service:backend-error (c)
          (handler:report-auth-error (service:backend-error-message c)))))))

(defparameter *get-logout*
  (lambda (params)
    (declare (ignore params))
    (handler:with-error-handler
      (handler-case
          (let ((token (get-cookie "token")))
            (logout token)
            (set-status 303)
            (set-header :location "/")
            (set-cookie "token"
                        :value ""
                        :path "/"
                        :secure nil
                        :httponly t
                        :samesite :strict
                        :expires 0))
        (service:backend-error (c)
          ;; just in case there is an invalid cookie in the request
          (set-cookie "token"
                      :value ""
                      :path "/"
                      :secure nil
                      :httponly t
                      :samesite :strict
                      :expires 0)
          (handler:report-error (:title (format nil "~a Error" (service:backend-error-status c))
                                 :message (service:backend-error-message c))))))))
