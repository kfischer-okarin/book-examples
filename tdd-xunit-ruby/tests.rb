require_relative 'xunit'

class WasRun < TestCase
  attr_reader :log

  def set_up
    @log = 'set_up '
  end

  def test_method
    @log += 'test_method '
  end
end

class TestCaseTest < TestCase
  def test_template_method
    test = WasRun.new('test_method')
    test.run
    assert! test.log == 'set_up test_method '
  end
end

TestCaseTest.new('test_template_method').run
