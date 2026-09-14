;; run with: sbcl --load build.lisp --disable-debugger
(push (truename ".") asdf:*central-registry*)
;; (ql:quickload "frontend")
(asdf:load-system "frontend")
(sb-ext:save-lisp-and-die "frontend"
                          :toplevel #'frontend:main
                          :executable t)
