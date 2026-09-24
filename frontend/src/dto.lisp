(uiop:define-package frontend.dto
  (:nicknames dto)
  (:use #:cl)
  (:export #:bad-input
           #:bad-input-reason)
  (:export #:params->id)
  (:export #:params->login-params
           #:params->register-params)
  (:export #:post-params-title
           #:post-params-content
           #:params->post-params)
  (:export #:pagination-params
           #:pagination-params-limit
           #:pagination-params-offset
           #:params->pagination-params)
  (:export #:params->comment-parent-type)
  (:export #:comment-params
           #:comment-params-parent
           #:comment-params-content
           #:params->comment-params)
  (:export #:sort-params
           #:sort-params-sort-by
           #:params->sort-params)
  (:export #:vote-params
           #:vote-params-id
           #:vote-params-from
           #:params->vote-params))

(in-package :frontend.dto)

(define-condition bad-input (error)
  ((reason :initarg :reason
           :initform nil
           :reader bad-input-reason)))

(defun params->id (params)
  "Returns an integer representing an item id from PARAMS if it exists,
otherwise signals a BAD-INPUT condition."
  (let* ((id-str (cdr (assoc :id params)))
         (id (when id-str
               (parse-integer id-str :junk-allowed t))))
    (unless id
      (error 'bad-input :reason "Missing ID."))
    id))

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

(defun params->comment-parent-type (params)
  (let ((parent-type (cdr (assoc "parent-type" params :test #'string=))))
    (unless (or (string= "post" parent-type)
                (string= "comment" parent-type))
      (error 'bad-input :reason "Missing or bad parent type."))
    parent-type))

(defstruct comment-params
  (parent nil :type integer)
  (content nil :type string))

(defun params->comment-params (params)
  (let* ((parent-str (cdr (assoc "parent" params :test #'string=)))
         (parent (when parent-str
                   (parse-integer parent-str :junk-allowed t)))
         (content (cdr (assoc "content" params :test #'string=))))
    (unless parent
      (error 'bad-input :reason "Missing parent id."))
    (unless content
      (error 'bad-input :reason "Missing content."))
    (make-comment-params :parent parent :content content)))

(defstruct sort-params
  (sort-by nil :type string))

(defun params->sort-params (params)
  (let ((sort-by (or (cdr (assoc "sort-by" params :test #'string=))
                     "oldest")))
    (unless (or (string= "oldest" sort-by)
                (string= "newest" sort-by))
      (error 'bad-input :reason "Bad parameter for sort-by."))
    (make-sort-params :sort-by sort-by)))

(defstruct vote-params
  (id nil :type integer)
  (from nil :type string))

(defun params->vote-params (params)
  (let* ((id-str (cdr (assoc "id" params :test #'string=)))
         (id (when id-str
               (parse-integer id-str :junk-allowed t)))
         (from (cdr (assoc "from" params :test #'string=))))
    (unless id
      (error 'bad-input :reason "Missing id."))
    (unless from
      (error 'bad-input :reason "Missing from page."))
    (make-vote-params :id id :from from)))
