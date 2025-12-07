{{ sink(name="postgres_sink") }}

SELECT 
    *
FROM {{ use_source("github_repos") }} AS t;
