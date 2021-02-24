import init, {create_element, del_element, test_dom} from '../rust-crate/pkg';

const start = async () => {
  await init();
  // create_element("div", "body")
  // del_element("h1")
  test_dom();
}

start()


