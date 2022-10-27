class TestCase
  class AssertionFailed < StandardError; end

  def initialize(name)
    @name = name
  end

  def set_up; end

  def run
    set_up
    send(@name)
  end

  def assert!(value)
    raise AssertionFailed, 'Assertion failed' unless value
  end
end
