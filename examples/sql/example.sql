{{ sink(name="postgres_sink") }}


select 
    * 
from {{ use_source("peopleforce_employees") }};

{{ scheduler("@daily") }}