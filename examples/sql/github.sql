{{ sink(name="postgres_sink") }}

SELECT 
    t.content['id'] as id,
    t.content['type'] as content_type,
    t.content['status'] as content_status,
    t.content['title'] as content_title,
    t.content['restrictions'] as content_restrictions,
    t.content['_links'] as content_links,
    t.content['_links']['webui'] as content_webui,
    t.content['_links']['self'] as content_self
FROM {{ use_source("confluence_content") }} AS t;
