main :: IO ()
main = do
  x <- readLn :: IO Integer
  print $ calculate x

calculate :: Integer -> Integer
calculate x
  | x `mod` 10 == 0 = x `div` 10
  | x >= 0 = (x `div` 10) + 1
  | otherwise = if abs x `mod` 10 == 0 then x `div` 10 else (x `div` 10) + 1
