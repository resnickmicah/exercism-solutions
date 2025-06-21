(ns bank-account)

(defn open-account [] ;; <- arglist goes here
  ;; your code goes here
  (atom {:balance 0 :open? true})
  )

(defn close-account [account] ;; <- arglist goes here
  (if (:open? @account) (swap! account assoc :open? false))
  )

(defn get-balance [account] ;; <- arglist goes here
  (let [acct @account]
    (if (:open? acct) (:balance acct)))
  )

(defn update-balance [account new-balance] ;; <- arglist goes here
  (if (:open? @account) (swap! account update :balance + new-balance))
  )
