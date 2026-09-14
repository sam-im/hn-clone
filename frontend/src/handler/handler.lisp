(uiop:define-package frontend.handler
  (:nicknames handler)
  (:use #:cl)
  (:export #:set-status
           #:set-header
           #:set-cookie
           #:set-body
           #:get-cookie))

(in-package #:frontend.handler)

(defmacro set-status (status)
  "Sets the status code of a ningle:*response*.
STATUS must be an integer representing a valid HTTP status code.
Returns NIL."
  `(progn
     (setf (lack.response:response-status ningle:*response*) ,status)
     nil))

(defmacro set-header (key value)
  "Sets an header on a response to KEY and VALUE.
KEY must be a keyword accepted by lack.response.
Returns NIL."
  `(progn
     (setf (getf (lack.response:response-headers ningle:*response*) ,key)
           ,value)
     nil))

(defmacro set-cookie (name &rest plist)
  "Sets a cookie on a response.
PLIST must be a property list of keywords and values accepted by lack.response.
Returns NIL."
  `(progn
     (setf (getf (lack.response:response-set-cookies ningle:*response*) ,name)
           (list ,@plist))
     nil))

(defmacro set-body (body)
  "Sets the body of a response to BODY.
Returns NIL."
  `(progn
     (setf (lack.response:response-body ningle:*response*) ,body)
     nil))

(defmacro get-cookie (name)
  "Retrieve the cookie specified by NAME."
  `(cdr (assoc ,name (lack.request:request-cookies ningle:*request*) :test #'string=)))
