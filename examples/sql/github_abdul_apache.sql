{{ sink(name="postgres_sink") }}
{{ schedule("0 */3 * * * *")  }}


SELECT 
    *
FROM {{ use_source("github_apache") }} AS t;
