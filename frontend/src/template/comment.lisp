(uiop:define-package #:frontend.template.comment
  (:use #:cl)
  (:import-from #:spinneret
                #:with-html)
  (:import-from #:frontend.template
                #:unix-to-timestamp)
  (:export #:render-comment))

(in-package #:frontend.template.comment)

(defun render-comment (comment replies &optional current-page)
  (with-html
    (:article
     :id (format nil "item-~a" (gethash "id" comment))
     (:h5 "Comment")                    ; TODO: consider adding an indication of what the parent is
     (:div
      (gethash "content" comment))
     (:div
      :style "padding-top: 1rem; font-size: 0.85rem;"
      (:span (gethash "owner" comment))
      (:span " · ")
      (multiple-value-bind (datetime time)
          (unix-to-timestamp (gethash "created_at" comment))
        (:time :datetime datetime time))
      (template:render-upvotes (gethash "id" comment)
                               (gethash "upvotes" comment)
                               (or current-page
                                   (format nil "/comment/~a" (gethash "id" comment))))
      (:div
       :style "display: inline-block; padding-left: 1rem;"
       (:a :href "#reply-section" (format nil "~a replies" (gethash "replies" comment))))
      (:div
       (:details
        :style "border: none; background: none; margin: 0; padding: 0;"
        (:summary :style "font-weight: normal;" "Write a reply")
        (:form
         :action "/comment" :method "post"
         (:input :type "hidden" :name "parent" :value (gethash "id" comment))
         (:input :type "hidden" :name "parent-type" :value "comment")
         (:textarea :name "content" :rows "3" :required t)
         (:button :type "submit" "Submit"))))))
    (:section
     :id "reply-section"
     (:div
      :style "display: flex; align-items: baseline; gap: 1rem;"
      (:h5 :style "margin: 0;" "Replies")
      (:a :href (format nil "/comment/~a" (gethash "id" comment))
          :style "font-size: 0.85rem;" "Oldest")
      (:a :href (format nil "/comment/~a?sort-by=newest" (gethash "id" comment))
          :style "font-size: 0.85rem;" "Newest"))
     (:div
      (dolist (reply (coerce (gethash "data" replies) 'list))
        (:div
         :id (format nil "item-~a" (gethash "id" reply))
         :style "border-bottom: var(--border-width) solid var(--border); padding-bottom: 1rem;"
         (:p (gethash "content" reply))
         (:div
          :style "font-size: 0.85rem;"
          (:div
           :style "display: flex;"
           (:div
            (:span (gethash "owner" reply))
            (:span " · ")
            (multiple-value-bind (datetime time)
                (unix-to-timestamp (gethash "created_at" reply))
              (:time :datetime datetime time)))
           (template:render-upvotes (gethash "id" reply)
                                    (gethash "upvotes" reply)
                                    (or current-page
                                        (format nil "/comment/~a" (gethash "id" comment))))
           (:div
            :style "padding-left: 1rem;"
            (:a :href (format nil "/comment/~a" (gethash "id" reply))
                (format nil "~a replies" (gethash "replies" reply)))))
          (:div
           (:details
            :style "border: none; background: none; margin: 0; padding: 0;"
            (:summary :style "font-weight: normal;" "Reply")
            (:form
             :action "/comment" :method "post"
             (:input :type "hidden" :name "parent" :value (gethash "id" reply))
             (:input :type "hidden" :name "parent-type" :value "comment")
             (:textarea :name "content" :rows "3" :required t)
             (:button :type "submit" "Submit"))))))))
     (template:render-pagination (gethash "offset" replies)
                                 (format nil "/comment/~a" (gethash "id" comment))))))
