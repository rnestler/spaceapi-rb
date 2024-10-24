# frozen_string_literal: true

require "test_helper"

class TestSpaceapi < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Spaceapi::VERSION
  end

  def test_it_does_something_useful
    assert_raises do
      Spaceapi.status_from_string("{}")
    end
  end
end
