module fastICA
    export correlation_matrix, covariance_matrix, eigen_decomposition, sing_val_decomp, whiten
    using LinearAlgebra
    using Statistics
    
    correlation_matrix(mat::Array{Float64 ,2})::Array{Float64 ,2} = (mat .- mean(mat,dims=1)) ./ std(mat,dims=1)   
    
    covariance_matrix(corrmat::Array{Float64 ,2})::Array{Float64 ,2} = corrmat |> length |> float |> n -> corrmat' * corrmat / (n  - 1.)

    sing_val_decomp(mat::Array{Float64 ,2})::SVD = mat |> correlation_matrix |> svd
    
    #Returns: a whitened matrix using one of the available whitening techniques(PCA or ZCA)
    #Receives: 
    #the matrix to whiten 
    #optional boolean which is used to determine the whitening technique
    function whiten(mat::Array{Float64 ,2},pca::Bool = true)::Array{Float64 ,2}
        U,S,_ = sing_val_decomp(mat)
        values = broadcast(+,  eps(0.3), S)
        if(pca)
            return (Diagonal(values .^ (-1/2)) * U') * mat
        end
        return U * (Diagonal(values .^ (-1/2)) * U') * mat 
    end

end