(defsystem "frontend"
  :version "0.0.1"
  :author "sam"
  :license "GPL-3.0-or-later"
  :depends-on ("woo"                    ; http server
               "ningle"                 ; (micro) web framework
               "com.inuoe.jzon"         ; json reader/writer
               "dexador"                ; http client
               "spinneret"              ; html templating
               "lack")
  :components ((:module "src"
                :components
                ((:file "dto")
                 (:module "template"
                  :components ((:file "template")
                               (:file "login")
                               (:file "register")))
                 (:module "service"
                  :components ((:file "service")
                               (:file "user")))
                 (:module "handler"
                  :components ((:file "handler")
                               (:file "login")
                               (:file "register")))
                 (:file "server")
                 (:file "main"))))
  :description ""
  :in-order-to ((test-op (test-op "frontend/tests"))))

(defsystem "frontend/tests"
  :author "sam"
  :license "GPL-3.0-or-later"
  :depends-on ("frontend"
               "fiveam")
  :components ((:module "tests"
                :components
                ((:file "main"))))
  :description ""
  :perform (test-op (op c) (symbol-call :fiveam :run! (find-symbol* :all-tests :frontend/tests/main))))

