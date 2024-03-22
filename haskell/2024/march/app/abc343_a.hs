main :: IO ()
main = do
  input <- getLine

  let [a, b] = map read $ words input :: [Int]

  let sum = a + b
  let vecSmp = [0 .. 9]

  case filter (/= sum) vecSmp of
    (x : _) -> print x
