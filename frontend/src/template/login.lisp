(uiop:define-package #:frontend.template.login
  (:use #:cl)
  (:import-from #:spinneret #:with-html)
  (:export #:render-login-form))

(in-package #:frontend.template.login)

;; TODO: add an optional hidden input element for a return page
(defun render-login-form ()
  (with-html
    (:fieldset
     (:legend "Login")
     (:form :action "/login" :method "post"
      (:label :for "username" "Username:")
      (:input :type "text" :placeholder "Username" :name "username" :required t)
      (:label :for "password" "Password:")
      (:input :type "password" :placeholder "Password" :name "password" :required t)
      (:div
       (:label :for "duration" "Session duration:")
       (:select :name "duration"
         (:option :selected t :value 60 "1 hour")
         (:option :value (* 8 60) "8 hours")
         (:option :value (* 24 60) "1 day")
         (:option :value (* 7 24 60) "1 week")))
      (:button :type "submit" "Login")))
    (:a :href "/register" "Register")))
