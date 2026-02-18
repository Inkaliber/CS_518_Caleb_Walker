(ns member_twice.core
  (:gen-class))

(defn member-twice?
  "True iff `x` appears at least twice in `coll`."
  [x coll]
  (boolean
    (some #(= x %)
          (next (drop-while #(not= x %) coll)))))

(defn -main [& _args]
  (println "member-twice? :a [:b :a :c :a] =>"
           (member-twice? :a [:b :a :c :a]))
  (println "member-twice? :a [:b :a :c] =>"
           (member-twice? :a [:b :a :c]))
  (println "member-twice? nil [1 nil 2 nil] =>"
           (member-twice? nil [1 nil 2 nil]))
  (println "member-twice? 5 [1 nil 2 5 nil 5] =>"
           (member-twice? 5 [1 nil 2 5 nil 5]))
  (println "member-twice? 4 [1 nil 2 5 nil 5] =>"
           (member-twice? 4 [1 nil 2 5 nil 5])))