import Control.Applicative

main = do
    (c:cs) <- getLine
    putStrLn $ show (cs ++ [c]) 
