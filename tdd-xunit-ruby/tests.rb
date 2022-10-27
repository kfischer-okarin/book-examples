require_relative 'xunit'

class WasRun < TestCase
  def initialize(name)
    super
    @was_run = false
    @was_set_up = false
  end

  def was_run?
    @was_run
  end

  def was_set_up?
    @was_set_up
  end

  def set_up
    @was_set_up = true
  end

  def test_method
    @was_run = true
  end
end

class TestCaseTest < TestCase
  def test_running
    test = WasRun.new('test_method')
    assert! !test.was_run?
    test.run
    assert! test.was_run?
  end

  def test_set_up
    test = WasRun.new('test_method')
    test.run
    assert! test.was_set_up?
  end
end

TestCaseTest.new('test_running').run
TestCaseTest.new('test_set_up').run
