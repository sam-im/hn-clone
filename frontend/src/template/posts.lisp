(uiop:define-package #:frontend.template.posts
  (:use #:cl)
  (:import-from #:spinneret #:with-html)
  (:export #:render-posts))

(in-package #:frontend.template.posts)

(defun render-posts (posts &optional (sort-by "popular") (sort-order "desc") (offset 0) (limit 20))
  (with-html
    (:div
     :style "display: flex; justify-content: space-between; align-items: baseline; gap: 1rem; flex-wrap: wrap;"
     (:form
      :action "/posts"
      :method "get"
      :style "display: flex; align-items: baseline; gap: 0.5rem; flex-wrap: wrap;"
      (:label :for "sort-by" :style "display: inline;" "Sort by:")
      (:select :name "sort-by" :style "width: auto;"
        (:option :selected (string= "popular" sort-by) :value "popular" "Popularity")
        (:option :selected (string= "vote" sort-by) :value "vote" "Upvotes")
        (:option :selected (string= "date" sort-by) :value "date" "Date"))
      (:label :for "sort-order" :style "display: inline; padding-left: 1rem;" "Order by:")
      (:select :name "sort-order" :style "width: auto;"
        (:option :selected (string= "desc" sort-order) :value "desc" "Descending")
        (:option :selected (string= "asc" sort-order) :value "asc" "Ascending"))
      (:button :type "submit" "Apply"))
     (:a :class "button" :href "/new-post" "Create post"))
    (:ol
     (dolist (post (coerce (gethash "data" posts) 'list))
       (:li
        (:a :href (format nil "/post/~a" (gethash "id" post))
            :style "font-size: 1.5rem;" (gethash "title" post))
        (:br)
        (:small
         (:a :href (format nil "/user/~a" (gethash "owner" post)) (gethash "owner" post))
         " · "
         (multiple-value-bind (datetime time)
             (template:unix-to-timestamp (gethash "created_at" post))
           (:time :datetime datetime time))
         (template:render-upvotes (gethash "id" post)
                                  (gethash "upvotes" post)
                                  (format nil "/posts?sort-by=~a&sort-order=~a&offset=~a&limit=~a"
                                          sort-by
                                          sort-order
                                          offset
                                          limit))
         (:a :href (format nil "/post/~a" (gethash "id" post))
             :style "padding-left: 1rem;"
             (format nil "~a comments" (gethash "comments" post)))))))
    (template:render-pagination (gethash "offset" posts) "/posts" (list (cons "sort-by" sort-by)
                                                                        (cons "sort-order" sort-order)))))
