(defsystem "frontend"
  :version "0.0.1"
  :author "sam"
  :license "GPL-3.0-or-later"
  :depends-on ()
  :components ((:module "src"
                :components
                ((:file "main"))))
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

