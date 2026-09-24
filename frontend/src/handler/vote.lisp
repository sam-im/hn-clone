(uiop:define-package frontend.handler.vote
  (:use #:cl)
  (:import-from :frontend.handler
                #:get-cookie)
  (:import-from :frontend.service.vote
                #:create-vote
                #:delete-vote
                #:toggle-vote)
  (:export #:*post-upvote*))

(in-package :frontend.handler.vote)

(defparameter *post-upvote*
  (lambda (params)
    (handler:with-error-handler
      (handler-case
          (let* ((token (get-cookie "token"))
                 (vote-params (dto:params->vote-params params)))
            (toggle-vote token (dto:vote-params-id vote-params))
            (handler:redirect (format nil "~a#item-~a"
                                      (dto:vote-params-from vote-params)
                                      (dto:vote-params-id vote-params))))
        (dto:bad-input (c)
          (handler:report-error (:title "Error" :message (dto:bad-input-reason c))))
        (service:backend-error (c)
          (let ((status (service:backend-error-status c)))
            (if (= status 401)
                (handler:report-auth-error (service:backend-error-message c))
                (handler:report-error (:title (if (= status 404)
                                                  "Item Not Found"
                                                  "Error")
                                       :message (service:backend-error-message c))))))))))
