(defproject borkdude/plsci "0.0.1-SNAPSHOT"
  :description "TODO"
  :url "https://github.com/borkdude/plsci"
  :scm {:name "git"
        :url "https://github.com/borkdude/plsci"}
  :license {:name "Eclipse Public License 1.0"
            :url "http://opensource.org/licenses/eclipse-1.0.php"}
  :source-paths ["src"]
  :target-path "libplsci"
  :compile-path "%s/classes/main"
  :dependencies [[borkdude/sci "0.2.0"]
                 [cheshire/cheshire "5.10.0"]
                 [org.clojure/clojure "1.10.2-rc1"]
                 [borkdude/sci.impl.reflector "0.0.1-jdk11"]
                 [borkdude/clj-reflector-graal-java11-fix "0.0.1-graalvm-20.3.0"]]
  :profiles {:uberjar {:global-vars {*assert* false}
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"
                                  "-Dclojure.spec.skip-macros=true"]
                       :aot :all
                       :main plsci.core}})

