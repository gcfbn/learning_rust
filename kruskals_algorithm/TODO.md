- usuń bail
- dodaj testy
- zdefiniuj errory dla biblioteki (użyj thiserror)
  anyhow - tylko w programie używającym lib.r (main.rs)
- przesuń pliki testowe *.txt* do katalogu tests/data
- zmień nazwę metody: tests/example_tests.rs: integration_test -> passing (dodaj metodę failing - z testami, które nie przechodzą)
- zmień nazwę pliku: tests/example_tests.rs -> tests/acceptance_tests.rs
- w testach graph_not_connected i graph_connected budujesz graph w postaci Vec<Edge> i korzystasz funkcji is_connected, a zrezygnowałeś z implementacji tych funkci opartych o GraphParameters i GraphBuilder - dlaczego ?
  Obie te funkcje mógłbyś zaimplementować przy pomocy: kruskals_algorithm::run(cmd_args.task_file( test_file_name ) 
