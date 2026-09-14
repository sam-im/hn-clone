(uiop:define-package frontend
  (:use #:cl)
  (:import-from :frontend.server #:make-app)
  (:export #:main))
(in-package #:frontend)

(defparameter *address* "127.0.0.1")
(defparameter *port* 5000)
(defparameter *worker-num* 4)


(defun main ()
  (let ((app (make-app)))
    (woo:run app :address *address*
                 :port *port*
                 :worker-num *worker-num*)))
