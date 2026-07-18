(defpackage frontend/tests/main
  (:use :cl
        :frontend
        :fiveam))
(in-package :frontend/tests/main)

(def-suite all-tests
  :description "The main suite of all frontend tests.")

(in-suite all-tests)

