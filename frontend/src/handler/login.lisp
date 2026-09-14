(uiop:define-package frontend.handler.login
  (:use #:cl)
  (:import-from :frontend.handler
                #:set-status #:set-header #:set-cookie #:set-body
                #:get-cookie)
  (:import-from :frontend.template
                #:with-page
                #:render-error-message)
  (:import-from :frontend.template.login
                #:render-login-form)
  (:import-from :frontend.service.user
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
    (set-body (with-page (:title "Login")
                (render-login-form)))))

(defparameter *post-login*
  (lambda (params)
    (flet ((report-error (msg)
             (set-status 200)
             (set-header :content-type "text/html; charset=utf-8")
             (set-body (with-page (:title "Login")
                         (render-error-message msg)
                         (render-login-form)))))
      (handler-case
          (let* ((form (dto:params->login-params params))
                 (response (login form))
                 (token (gethash "token" response))
                 (expires (+ (get-universal-time) (- (gethash "expires_at" response)
                                                     (gethash "issued_at" response)))))
              (set-status 303)
              (set-header :location "/")
              (set-cookie "token" :value token
                                  :path "/"
                                  :secure nil ; set to T if https
                                  :httponly t
                                  :samesite :strict
                                  :expires expires))
        (dto:bad-input (c)
          (report-error (dto:bad-input-reason c)))
        (service:backend-error (c)
          (report-error (service:backend-error-message c)))))))

(defparameter *get-logout*
  (lambda (params)
    (declare (ignore params))
    (handler-case
        (let ((token (get-cookie "token")))
          (logout token)
          (set-status 303)
          (set-header :location "/")
          (set-cookie "token" :value ""
                              :path "/"
                              :secure nil ; set to T if https
                              :httponly t
                              :samesite :strict
                              :expires 0))

      (service:backend-error (c)
        (set-status 200)
        (set-header :content-type "text/html; charset=utf-8")
        ;; there may be an invalid session cookie
        (set-cookie "token" :value ""
                            :path "/"
                            :secure nil ; set to T if https
                            :httponly t
                            :samesite :strict
                            :expires 0)
        (set-body (with-page (:title "Error")
                    (render-error-message (service:backend-error-message c))))))))
