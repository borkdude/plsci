(ns plsci.core
  (:require [cheshire.core :as cheshire]
            [sci.core :as sci])
  (:gen-class
   :methods [^{:static true} [evalString [String] String]]))

(defn -evalString [s]
  (sci/binding [sci/out *out*] ;; this enables println etc.
    (str (try (sci/eval-string
               s
               ;; this brings cheshire.core into sci
               {:namespaces {'cheshire.core {'generate-string cheshire/generate-string}}})
              (catch Exception e
                {:error (str (type e))
                 :message (.getMessage e)})))))
