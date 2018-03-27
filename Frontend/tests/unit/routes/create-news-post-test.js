import { module, test } from 'qunit';
import { setupTest } from 'ember-qunit';

module('Unit | Route | create-news-post', function(hooks) {
  setupTest(hooks);

  test('it exists', function(assert) {
    let route = this.owner.lookup('route:create-news-post');
    assert.ok(route);
  });
});
