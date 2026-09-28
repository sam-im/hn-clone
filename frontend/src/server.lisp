(uiop:define-package frontend.server
  (:use #:cl)
  (:import-from :frontend.handler.login
                #:*get-login*
                #:*post-login*
                #:*get-logout*)
  (:import-from :frontend.handler.register
                #:*get-register*
                #:*post-register*)
  (:import-from :frontend.handler.post
                #:*get-post*
                #:*get-new-post*
                #:*post-post*)
  (:import-from :frontend.handler.comment
                #:*get-comment*
                #:*post-comment*)
  (:import-from :frontend.handler.vote
                #:*post-upvote*)
  (:import-from :frontend.handler.posts
                #:*get-posts*)
  (:export #:make-app))

(in-package #:frontend.server)

(defun make-app ()
  (let ((app (make-instance 'ningle:app)))
    ;; (setf (ningle:route app "/") *get-root*)
    (setf (ningle:route app "/login") *get-login*)
    (setf (ningle:route app "/login" :method :post) *post-login*)
    (setf (ningle:route app "/logout") *get-logout*)
    (setf (ningle:route app "/register") *get-register*)
    (setf (ningle:route app "/register" :method :post) *post-register*)
    (setf (ningle:route app "/post/:id") *get-post*)
    (setf (ningle:route app "/new-post") *get-new-post*)
    (setf (ningle:route app "/post" :method :post) *post-post*)
    (setf (ningle:route app "/comment/:id") *get-comment*)
    (setf (ningle:route app "/comment" :method :post) *post-comment*)
    (setf (ningle:route app "/upvote" :method :post) *post-upvote*)
    (setf (ningle:route app "/posts") *get-posts*)
    (lack:builder
     (:static :path "/static/" :root (asdf:system-relative-pathname :frontend "static/"))
     app)))
