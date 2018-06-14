import { module, test } from 'qunit';
import { setupTest } from 'ember-qunit';

module('Unit | Route | home/forums-listings', function(hooks) {
  setupTest(hooks);

  test('it exists', function(assert) {
    let route = this.owner.lookup('route:home/forums-listings');
    assert.ok(route);
  });
});
