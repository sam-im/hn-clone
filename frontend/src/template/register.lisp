(uiop:define-package #:frontend.template.register
  (:use #:cl)
  (:import-from #:spinneret #:with-html)
  (:export #:render-register-form))

(in-package #:frontend.template.register)

(defun render-register-form ()
  (with-html
    (:form
     :action "/register" :method "post"
     (:div
      (:div (:label :for "username" "Username:")
            (:input :type "text" :placeholder "Username" :name "username" :required t))
      (:div (:label :for "password" "Password:")
            (:input :type "password" :placeholder "Password" :name "password" :required t))
      (:div (:label :for "re-password" "Retype password:")
            (:input :type "password" :placeholder "Password" :name "re-password" :required t))
      (:button :type "submit" "Register")))))
