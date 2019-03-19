module fastICA
    export correlation_matrix, covariance_matrix, eigen_decomposition, sing_val_decomp, whiten
    using LinearAlgebra
    using Statistics
    
    struct ICA
    mean::Vector{Float64,1}
    W::Array{Float64,2}
    end

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
        if(pca)
            return (Diagonal(values .^ (-1/2)) * U') * mat
        end
        return U * (Diagonal(values .^ (-1/2)) * U') * mat 
    end
    
    #Returns: the resultant ICA model, an instance of the type ICA
    #Receives:
    #maxiter -> number of iterations for the main loop
    #nic     -> number of independent components
    #X       -> the data matrix, must be whitened(use the whiten function)
    #tol     -> tolerable change of the weights at convergence
    function fast_ica(maxiter::Int32, nic::Int8,X::Array{Float64,2},tol::Float64)::ICA
        w = rand(nic); # initialize weights with random values
        
    end
end