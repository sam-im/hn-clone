(uiop:define-package frontend.template.post
  (:use #:cl)
  (:import-from :spinneret
                #:with-html)
  (:import-from :frontend.template
                #:unix-to-timestamp)
  (:export #:render-post
           #:render-new-post))

(in-package #:frontend.template.post)

(defun render-post (post comments)
  (with-html
    ;; post
    (:article
     :id (format nil "post-~a" (gethash "id" post))
     (:h3 (gethash "title" post))
     (:div
      (gethash "content" post))
     (:div
      :style "padding-top: 1rem; font-size: 0.85rem;"
      (:span (gethash "owner" post))
      (:span " · ")
      (multiple-value-bind (datetime time)
          (unix-to-timestamp (gethash "created_at" post))
        (:time :datetime datetime time))
      (template:render-upvotes (gethash "id" post) (gethash "upvotes" post))
      (:div
       :style "display: inline-block; padding-left: 1rem;"
       (:a :href "#comment-section" (format nil "~a comments" (gethash "comments" post))))
      (:div
       (:details
        :style "border: none; background: none; margin: 0; padding: 0;"
        (:summary :style "font-weight: normal;" "Write a comment")
        (:form
         :action "/comment" :method "post"
         (:input :type "hidden" :name "parent" :value (gethash "id" post))
         (:input :type "hidden" :name "parent-type" :value "post")
         (:textarea :name "content" :rows "3" :required t)
         (:button :type "submit" "Submit"))))))
    ;; comments
    (:section
     :id "comment-section"
     (:div
      :style "display: flex; align-items: baseline; gap: 1rem;"
      (:h5 :style "margin: 0;" "Comments")
      (:a :href (format nil "/post/~a" (gethash "id" post)) :style "font-size: 0.85rem;" "Oldest")
      (:a :href (format nil "/post/~a?sort-by=newest" (gethash "id" post))
          :style "font-size: 0.85rem;" "Newest"))
     (:div
      (dolist (comment (coerce (gethash "data" comments) 'list))
        (:div
         :id (format nil "comment-~a" (gethash "id" comment))
         :style "border-bottom: var(--border-width) solid var(--border); padding-bottom: 1rem;"
         (:p (gethash "content" comment))
         (:div
          :style "font-size: 0.85rem;"
          (:div
           :style "display: flex;"
           (:div
            (:span (gethash "owner" comment))
            (:span " · ")
            (multiple-value-bind (datetime time)
                (unix-to-timestamp (gethash "created_at" comment))
              (:time :datetime datetime time)))
           (template:render-upvotes (gethash "id" comment) (gethash "upvotes" comment))
           (:div
            :style "padding-left: 1rem;"
            (:a :href (format nil "/comment/~a" (gethash "id" comment))
                (format nil "~a replies" (gethash "replies" comment)))))
          (:div
           (:details
            :style "border: none; background: none; margin: 0; padding: 0;"
            (:summary :style "font-weight: normal;" "Reply")
            (:form
             :action "/comment" :method "post"
             (:input :type "hidden" :name "parent" :value (gethash "id" comment))
             (:input :type "hidden" :name "parent-type" :value "comment")
             (:textarea :name "content" :rows "3" :required t)
             (:button :type "submit" "Submit"))))))))
     (template:render-pagination (gethash "offset" comments)
                                 (format nil "/post/~a" (gethash "id" post))))))

(defun render-new-post (&optional title content)
  (with-html
    (:fieldset
     (:legend "New Post")
     (:form :action "/post" :method "post"
      (:label :for "title" "Title")
      (:input :type "text" :name "title" :placeholder "Enter a title" :required t
              :style "width: 100%;" title)
      (:label :for "content" "Content")
      (:textarea :name "content" :rows "24" :placeholder "Enter your post" content)
      (:button :type "submit" "Publish Post")))))
