import Data.List (findIndex)

main :: IO ()
main = do
  s <- getLine
  putStrLn $ show $ solve s

solve :: String -> Int
solve s
  | length s < 3 = error "invalid input"
  | head s == s !! 1 = 1 + maybe 2 (+ 3) (findIndex (/= head s) (drop 2 s))
  | otherwise = if head s == s !! 2 then 2 else 1