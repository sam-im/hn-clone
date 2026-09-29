(uiop:define-package #:frontend.handler.root
  (:use #:cl)
  (:import-from #:frontend.handler
                #:set-status #:set-header)
  (:export #:*get-root*))

(in-package #:frontend.handler.root)

(defparameter *get-root*
  (lambda (params)
    (declare (ignore params))
    (handler:redirect "/posts")))
