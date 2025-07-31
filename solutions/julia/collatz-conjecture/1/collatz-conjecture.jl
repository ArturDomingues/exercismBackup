function collatz_steps(n::Int)
    if n < 1
        throw(DomainError(n, "Only positive values"))
    end
    steps = 0
    while n > 1
        if iseven(n)
            n=n/2
            steps+=1
        else
            n=3n+1
            steps+=1
        end
    end
    return(steps)
end 