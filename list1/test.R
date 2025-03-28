# Load data from a text file (assuming one integer per line)
data <- scan("out.txt")

# Define range (0 to 100, inclusive)
max_k <- 99
values <- 0:max_k

# Uniform
unif_pmf <- rep(1/(max_k + 1), max_k + 1)

# Geometric(1/2) PMF (P(X = k) = (1 - p)^k * p)
p_geom <- 0.5
geo_pmf <- dgeom(values, p_geom)
geo_pmf <- geo_pmf / sum(geo_pmf)  # Normalize to sum to 1

# Harmonic distribution PMF (P(X = k) ∝ 1 / (k + 1))
harm_pmf <- 1 / (values + 1)
harm_pmf <- harm_pmf / sum(harm_pmf)  # Normalize

# Biharmonic distribution PMF (P(X = k) ∝ 1 / (k + 1)^2)
biharm_pmf <- 1 / (values + 1)^2
biharm_pmf <- biharm_pmf / sum(biharm_pmf)  # Normalize

# Create observed frequency table
obs_freq <- table(factor(data, levels = values))

# Perform Chi-Square Goodness-of-Fit tests
unif_test <- chisq.test(obs_freq, p = unif_pmf)
geo_test <- chisq.test(obs_freq, p = geo_pmf)
harm_test <- chisq.test(obs_freq, p = harm_pmf)
biharm_test <- chisq.test(obs_freq, p = biharm_pmf)

# Display results
print("Uniform Test:")
print(unif_test)

print("Geometric(1/2) Test:")
print(geo_test)

print("Harmonic Test:")
print(harm_test)

print("Biharmonic Test:")
print(biharm_test)

