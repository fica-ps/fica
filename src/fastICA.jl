module fastICA
    export correlation_matrix, covariance_matrix, eigen_decomposition, sing_val_decomp, whiten
    using LinearAlgebra
    using Statistics
    

    correlation_matrix(mat::Array{Float64 ,2})::Array{Float64 ,2} = (mat .- mean(mat,dims=1)) ./ std(mat,dims=1)   
    
    covariance_matrix(corrmat::Array{Float64 ,2})::Array{Float64 ,2} = corrmat |> length |> float |> n -> corrmat' * corrmat / (n  - 1.)

    sing_val_decomp(mat::Array{Float64 ,2})::SVD = mat |> correlation_matrix |> svd
    
    #Returns: a whitened matrix using one of the available whitening techniques(PCA or ZCA)
    #Receives: 
    #mat -> the matrix to whiten 
    #pca -> optional boolean which is used to determine the whitening technique
    function whiten(mat::Array{Float64 ,2},pca::Bool = true)::Array{Float64 ,2}
        U,S,_ = sing_val_decomp(mat)
        values = broadcast(+,  eps(0.3), S)
        pca_w = (Diagonal(values .^ (-1/2)) * U') * mat
        if(pca)
            return pca_w
        end
        #zca whitening
        return U * pca_w
    end 
    
    
    
    #Returns: the resultant ICA model, an instance of the type ICA
    #Receives:
    #maxiter -> number of iterations for the main loop
    #nic     -> number of independent components
    #X       -> the data matrix, must be whitened(use the whiten function)
    #tol     -> tolerable change of the weights at convergence
    function fast_ica(maxiter::Int32, nic::Int8,X::Array{Float64,2},tol::Float64)::Array{Float64 , 2}
        #validate arguments
        n,m = size(X)
        n > 1 || error("There must be more than one samples, n > 1.")
        maxiter > 1 || error("maxiter must be greater than 1.")
        tol > 0 || error("tol must be positive.")
        
        # initialize weights of size n with random values
        #src : https://en.wikipedia.org/wiki/FastICA 
        W = rand(n,nic)
        #old W
        oldW = Array{Float64}(undef,n,nic)
        #normalize weight vector to unity i.e. vector sum = 1
        #src : http://www.measurement.sk/2011/Patil.pdf pg 119 
        for i = 1:nic
            wp = view(W, :, nic)
            wp = normalize!(w,1)
        end
    
        #main loop
        # src: 
        #http://www.measurement.sk/2011/Patil.pdf pg 119 Fixed Point algorithm for ICA
        #https://www.cs.helsinki.fi/u/ahyvarin/papers/NN00new.pdf pg 15
        ones_col = ones(m)
        t = 0
        chg = 0
        converge = false
        while !converge && t < maxiter
            t+=1
            #store previous iteration W
            oldW = copyto!(oldW,W)
            for p = 1:nic
                #get a weight vector
                wp=view(W, :, p)
                # 1/M * X * g(wp' * X) ' - 1/M * g'(wp'X) *ones_col * wp 
                #src: https://en.wikipedia.org/wiki/FastICA 
                wp = 1/m * X * tanh.(wp' * X )' - 1/M * (1 - (tanh.(wp' * X)).^2) * ones_col * wp    
            end
            
            #symmetric decorrelation
            #src : https://www.cs.helsinki.fi/u/ahyvarin/papers/NN00new.pdf pg 15
            #eg : https://github.com/JuliaStats/MultivariateStats.jl/blob/master/src/ica.jl ln 120
            copyto!(W, W * (W'W).^(-1/2))     
            
            #check for convergence
            #eg: https://github.com/JuliaStats/MultivariateStats.jl/blob/master/src/ica.jl#L97 ln 124 and 125
            chg = maximum(abs.(abs.(diag(W*oldW')) .- 1.0))
            converge = ( chg < tol )
        end
        converge || throw("Convergence was not possible with tol = $tol, maxiter= $maxiter" 
        return W
    end
end