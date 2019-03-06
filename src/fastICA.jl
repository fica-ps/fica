module fastICA
    export correlation_matrix, covariance_matrix, eigen_decomposition, whiten_data
    using LinearAlgebra
    using Statistics
    
    correlation_matrix(mat::Array{Float64 ,2})::Array{Float64 ,2} = (mat .- mean(mat,dims=1)) ./ std(mat,dims=1)   
    
    covariance_matrix(corrmat::Array{Float64 ,2})::Array{Float64 ,2} = corrmat |> length |> float |> n -> corrmat' * corrmat / (n  - 1.)

    eigen_decomposition(mat::Array{Float64 ,2})::Eigen = mat |> correlation_matrix |> covariance_matrix |>eigen

    function whiten_data(mat::Array{Float64 ,2})::Array{Float64 ,2}
        eig = eigen_decomposition(mat)
        values = broadcast(+,  eps(0.3), eig.values)
        return (Diagonal(values .^ (-1/2)) * eig.vectors') * mat
    end

end