(uiop:define-package frontend.dto
  (:nicknames dto)
  (:use #:cl)
  (:export #:bad-input
           #:bad-input-reason)
  (:export #:params->login-params
           #:params->register-params)
  (:export #:params->post-id)
  (:export #:post-params-title
           #:post-params-content
           #:params->post-params)
  (:export #:pagination-params
           #:pagination-params-limit
           #:pagination-params-offset
           #:params->pagination-params))

(in-package :frontend.dto)

(define-condition bad-input (error)
  ((reason :initarg :reason
           :initform nil
           :reader bad-input-reason)))

(defstruct login-params
  (username nil :type string)
  (password nil :type string)
  (duration nil :type integer))

(defun params->login-params (params)
  (let* ((username (cdr (assoc "username" params :test #'string=)))
         (password (cdr (assoc "password" params :test #'string=)))
         (duration-str (cdr (assoc "duration" params :test #'string=)))
         (duration (or (when duration-str
                         (parse-integer duration-str :junk-allowed t))
                       60)))
    (unless (and (stringp username) (stringp password))
      (error 'bad-input :reason "Missing username and/or password."))
    (make-login-params :username username
                       :password password
                       :duration duration)))

(defstruct register-params
  (username nil :type string)
  (password nil :type string))

(defun params->register-params (params)
  (let ((username (cdr (assoc "username" params :test #'string=)))
        (password (cdr (assoc "password" params :test #'string=)))
        (re-password (cdr (assoc "re-password" params :test #'string=))))
    (unless (and (stringp username) (stringp password))
      (error 'bad-input :reason "Missing username and/or password."))
    (unless (string= password re-password)
      (error 'bad-input :reason "Passwords do not match."))
    (make-register-params :username username :password password)))

(defun params->post-id (params)
  "Extracts and returns a post id from the association list PARAMS.
Returns an integer representing a post id if it exists,
otherwise signals a BAD-INPUT condition."
  (let* ((post-id-str (cdr (assoc :id params)))
         (post-id (when post-id-str
                    (parse-integer post-id-str :junk-allowed t))))
    (unless post-id
      (error 'bad-input :reason "Missing post ID."))
    post-id))

(defstruct post-params
  (title nil :type string)
  (content nil :type string))

(defun params->post-params (params)
  (let ((title (cdr (assoc "title" params :test #'string=)))
        (content (or (cdr (assoc "content" params :test #'string=)) "")))
    (unless title
      (error 'bad-input :reason "Missing post title."))
    (make-post-params :title title :content content)))

(defstruct pagination-params
  (offset nil :type integer)
  (limit nil :type integer))

(defun params->pagination-params (params)
  (let* ((offset-str (cdr (assoc "offset" params :test #'string=)))
         (offset (or (when offset-str
                       (parse-integer offset-str :junk-allowed t))
                     0))
         (limit-str (cdr (assoc "limit" params :test #'string=)))
         (limit (or (when limit-str
                      (parse-integer limit-str :junk-allowed t))
                    20)))
    (make-pagination-params :offset offset :limit limit)))
